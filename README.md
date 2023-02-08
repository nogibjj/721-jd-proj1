# 721 Project 1

### Overview
A command-line tool that provides statistics on tennis player's career and head-to-head statistics between two players.


### Architecture 
Data on all men's professional tennis matches (ATP tour) between 1991-2016 comes from https://datahub.io/sports-data/atp-world-tour-tennis-data. 

<img width="644" alt="image" src="https://user-images.githubusercontent.com/86393045/217547567-4088a260-4e09-4f4b-820d-53f8ba024e57.png">




### Next Steps 
- incorporate a machine learning model that would predict the probability of win based on previous match-ups between two players (this was not completed as working with polars and reading in a csv was harder than expected)
- if data existed that was more readily updated, build a more dynamic pipeline that accesses the data via API or web scraping so the data where the stats and model are based upon are not static

### Benchmarking 




### Running 
Build: `cd` into `cli-proj` and run `make all`


Run to get a player's stats: `cargo run stats --player "player-name' --year 'year'` 


Run to get stats for match-ups between two players: `cargo run stats --player "player1-name' --opponent "player2-name"`
