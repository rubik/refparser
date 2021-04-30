download:
    wget -q https://s3-eu-west-1.amazonaws.com/snowplow-hosted-assets/third-party/referer-parser/referers-latest.json -O data/referers-latest.json

f:
    rustfmt $(find src -name "*.rs" -type f)
