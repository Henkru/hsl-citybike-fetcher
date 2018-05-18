#!/usr/bin/env python3
import logging
import json
import geohash
from graphqlclient import GraphQLClient
from influxdb import InfluxDBClient

DEFAULT_URL = 'https://api.digitransit.fi/routing/v1/routers/hsl/index/graphql'

def memoize(f):
    memo = {}
    def helper(x):
        if x not in memo:
            memo[x] = f(x)
        return memo[x]
    return helper

@memoize
def coord2geohash(p):
    (lat, lon) = p
    return geohash.encode(lat, lon)

class Fetcher:
    def __init__(self, url):
        self.client = GraphQLClient(url)
        logging.info("[Fetcher] init, using API URL %s", url)

    def fetch(self):
        try:
            data = json.loads(self.client.execute('''
                {
                  bikeRentalStations {
                    stationId
                    name
                    bikesAvailable
                    spacesAvailable
                    lat
                    lon
                  }
                }
            '''))
            return data['data']['bikeRentalStations']
        except Exception as e:
            logging.exception("[Fetcher] error: %s", str(e))
            return []

class Importer:
    def __init__(self, config):
        self.config = config
        self.client = InfluxDBClient(config['url'], config['port'], config['user'], config['password'], config['database'])
        logging.info("[Importer] Connecting InfluxDB: %s:%i", config['url'], config['port'])

    def save(self, data):
        def convert(station):
            geohash = coord2geohash((station['lat'], station['lon']))
            return {
                "measurement": "station",
                "tags": {
                    "name": station["name"],
                    "stationId": station["stationId"],
                    "geohash": geohash,
                    },
                "fields": {
                    "bikesAvailable": station["bikesAvailable"],
                    "spacesAvailable": station["spacesAvailable"],
                    }
            }

        data = list(map(convert, data))
        self.client.write_points(data)

def main():
    import schedule
    import os
    import time
    URL = os.environ.get('HSL_API', DEFAULT_URL)
    INTERVAL = int(os.environ.get('INTERVAL', '5'))
    config = {
        'url': os.environ.get('INFLUX_URL', 'localhost'),
        'port': int(os.environ.get('INFLUX_PORT', '8086')),
        'user': os.environ.get('INFLUX_USER', 'admin'),
        'password': os.environ.get('INFLUX_PASSWORD', 'admin'),
        'database': os.environ.get('INFLUX_DATABASE', 'test'),
    }

    if 'DEBUG' in os.environ:
        logging.basicConfig(level=logging.DEBUG)

    fetcher = Fetcher(URL)
    importer = Importer(config)

    def update():
        d = fetcher.fetch()
        importer.save(d)
        logging.debug("[Scheduler] updated stations")

    schedule.every(INTERVAL).minutes.do(update)
    logging.info("[Scheduler] interval: %i minute(s)", INTERVAL)
    update()
    while True:
        schedule.run_pending()
        time.sleep(60)

if __name__ == "__main__":
    main()
