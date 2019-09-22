# Pants

A simple webapp written with [Rocket](https://rocket.rs) for saving anything to Pocket.

## Setup
### Adjust the config file
todo

### Deploy
todo

## The problem
I find myself liking Pocket more and more, and I want to start using it to keep track of everything I want to read and have read. Unfortunately, not everything I want to read really fits Pocket's reading model, even if the tracking model is great. It doesn't really make sense to save The Hobbit to Pocket for example since there is no dedicated webpage containing that content.

## The solution
You can see in [the docs](https://getpocket.com/developer/docs/v3/add) for Pocket's add API that a URL is required, so we need to make one.

Let's say I want to read Big Boi's Bakery, by Big Boi, I can go to this webpage and enter the title (and optionally some tags) and then Pants will go and make a webpage under `/pages`, in this case `/pages/big-bois-bakery`, and then add it to Pocket.
