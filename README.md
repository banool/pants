# Pants

A simple webapp written with [Rocket](https://rocket.rs) for saving anything to Pocket. It's a lil Pocket Rocket hehehehehehehehh 🚀🚀🚀.

## Setup
### Get Pocket access token
First, make an app here: https://getpocket.com/developer/apps/new. It should give you a consumer key once you do this.

Now, to get a code for your Pocket user specifically, do this. Pretend our consumer key is `87796-6bf78d700933260a0f9b1b97`.
```
curl -d "consumer_key=87796-6bf78d700933260a0f9b1b97&redirect_uri=https://google.com" -X POST https://getpocket.com/v3/oauth/request
```
You should see a response like this:
```
code=07f3123f-0a12-d2f6-07e8-abcdef
```
Now you can authorize your app for your Pocket by visting this link:
```
https://getpocket.com/auth/authorize?request_token=07f3123f-0a12-d2f6-07e8-abcdef&redirect_uri=https://www.google.com
```
Now finally get the authorization token:
```
curl https://getpocket.com/v3/oauth/authorize -X POST -H "Content-Type: application/json" -H "X-Accept: application/json" -d "{\"consumer_key\":\"87796-6b548d766433260a0f9b1b97\",\"code\":\"07f3123f-0a12-d2f6-07e8-abcdef\"}"
```
The response should look like this:
```
{"access_token":"d44c7cc9-2434-de18-332c-923ad5","username":"danielporteous1@gmail.com"}
```
This is the token you need to set for the `PANTS_ACCESS_TOKEN` env var below

## Running locally
This assumes you have server-setup checked out above this directory.
```
./run.sh
```

## Running with docker
Build:
```
docker build . -t pants
```
Run:
```
docker run -e PANTS_PAGES_ROOT="/pages" -e PANTS_SITE="0.0.0.0" -e PANTS_CONSUMER_KEY="87796-6bf78d700933260a0f9b1b97" -e PANTS_ACCESS_TOKEN="d48c7af9-2450-de18-3e9c-9065f5" -e PANTS_ROCKET_SECRET_KEY="Ebbudtp1ttI8NMwDc54kQB1tStVV7KVp7L9Q1UTZ8/U=" -e PANTS_STATIC_ROOT="/var/static" --publish 8000:8000 -v /tmp/pantspages:/pages -it pants
```

## To publish
```
# Change the version in Cargo.toml
cargo publish
```

## The problem
I find myself liking Pocket more and more, and I want to start using it to keep track of everything I want to read and have read. Unfortunately, not everything I want to read really fits Pocket's reading model, even if the tracking model is great. It doesn't really make sense to save The Hobbit to Pocket for example since there is no dedicated webpage containing that content.

## The solution
You can see in [the docs](https://getpocket.com/developer/docs/v3/add) for Pocket's add API that a URL is required, so we need to make one.

Let's say I want to read Big Boi's Bakery, by Big Boi, I can go to Pants and enter the title (and optionally some tags) and then Pants will go and make a webpage under `/pages`, in this case `/pages/bigboisbakery`, and then add it to Pocket.
