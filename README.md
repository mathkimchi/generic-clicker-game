# Generic Clicker Game

A generic clicker web game made with rust.

I keep thinking of gameplay that I want to include, but I am going to keep this project true to the spirit of generic clicker games.

## Mechanics

### Dreams

Dreams will temporarily increase productivity when working. Working uses up your dreams.

You need sleepiness and a dream catcher to dream. Sleepiness will increase with any click you make. You can also increase your sleepiness with pills. You can dream without sleeping with a dream stealer, but that is more late stage.

If you get enough dreams, you can do the ecquivalent to prestigig by waking up.
Waking up will make you awakened, and you gain lucidity from this, which you can use for permanent upgrades.

If your sleep gets too low or you click on the toilet button while spam clicking, you will wake up from a bad dream and your progress will be lost and you lose some lucidity.
There will be upgrades early game to prevent this.

## TODO

- [x] Add incrementation (the main gameplay)
- [ ] Add dreams
- [ ] Afk income
- [ ] Check if the game is thread safe
- [ ] Figure out what to do with max int limit
- [ ] Add settings page
- [x] Autosave
- [ ] Graphics (just images)
- [x] Upgrades
- [ ] Prevent people from manually changing localstorage
  - It is a singleplayer game so it doesn't matter, but still interesting
- [ ] Google analytics
  - I don't want to track users and sell their info to google or whatever, but it would be cool to see how many people visited.
