import pandas as pd
import numpy as np

# read data 
stats = pd.read_csv("data/match_stats_1991-2016_unindexed_csv.csv")
scores = pd.read_csv("data/match_scores_1991-2016_unindexed_csv.csv")

scores_stats = scores.merge(stats.drop(columns='tourney_order').drop_duplicates(), on='match_id', how='inner')

# take out duplicate match ids 
duplicate_match_ids = scores_stats[scores_stats[['match_id']].duplicated()][['match_id']]

new_stats = stats[~stats['match_id'].isin(duplicate_match_ids['match_id'])]
new_scores = scores[~scores['match_id'].isin(duplicate_match_ids['match_id'])]

data = new_stats.set_index('match_id').join(new_scores.set_index('match_id'),
                                            how = "inner",lsuffix="stats",rsuffix="scores")
data['year'] = data['tourney_year_id'].astype(str).str[:4]

# process winner/ loser columns 
winner_col = [col for col in data if col.startswith('winner_')]
winners = data[winner_col + ['year', 'tourney_orderstats', 'round_order', 'match_order']].copy()
winners.columns = winners.columns.str.lstrip('winner_')
winners.loc[:,"win"] = 1
winners.reset_index(inplace=True)

loser_col = [col for col in data if col.startswith('loser_')]
losers = data[loser_col + ['year', 'tourney_orderstats', 'round_order', 'match_order']].copy()
losers.columns = losers.columns.str.lstrip('loser_')
losers.loc[:,"win"] = 0
losers.reset_index(inplace=True)
losers.columns = winners.columns

# make full data 
full_data = pd.concat([winners,losers], axis=0)
full_data.loc[:,"match_duration"] = np.repeat(list(data.match_duration),2)
full_data.reset_index(drop=True,inplace = True)
full_data.rename(columns={"ame": "name", "ound_order":'round_order'},inplace = True)


# make avgs 
avg = full_data.sort_values(['year', 'tourney_orderstats', 'round_order', 'match_order'], 
                            ascending=[True, True, False, True]).groupby("name").expanding().mean()
avg = avg.add_prefix('avg_')
avgs_win = avg.reset_index().set_index("level_1").join(full_data.loc[:,["win", 'match_id']])

names = avgs_win.groupby("name").size()
names = pd.DataFrame(names,columns = ["rows"])
names_high = names[names.rows >= 10]

avgs_win_sub = avgs_win[avgs_win.name.isin(list(names_high.index))]
avgs_win_sub = avgs_win.reset_index(drop = True)

move_avg = full_data.sort_values(['year', 'tourney_orderstats', 'round_order', 'match_order'], 
                                 ascending=[True, True, False, True]).groupby("name").rolling(5).mean()
move_avg = move_avg.add_prefix('move_avg_')

move_avg = full_data.sort_values(['year', 'tourney_orderstats', 'round_order', 'match_order'], 
                                    ascending=[True,True, False, True]).groupby("name").rolling(5).mean()
move_avg = move_avg.add_prefix('move_avg_')
move_avg = move_avg.dropna()
move_avg_win = move_avg.reset_index().set_index("level_1").join(full_data.loc[:,["match_id"]])
move_avg_win = move_avg_win.reset_index(drop = True)

move_mi = move_avg_win.match_id.drop_duplicates()

# find h2h 
scores_df = new_scores.copy()
scores_df['year'] = scores_df['tourney_year_id'].astype(str).str[:4]
scores_df = scores_df[test.match_id.isin(move_mi)].reset_index(drop=True)

scores_red = scores_df[['year', 'match_id', 'tourney_order', 'tourney_round_name', '
                        round_order', 'match_order', 'winner_name', 'loser_name']]
                        .sort_values(['year', 'tourney_order', 'round_order', 'match_order'], 
                                     ascending=[True, True, False, True])
                        
# make columns
scores_red['player1'] = np.where(scores_red['winner_name'] < scores_red['loser_name'], 
                                 scores_red['winner_name'], None)
scores_red['win'] = np.where(scores_red['player1'].isnull(), 0, 1)
scores_red['player2'] = np.where(scores_red['player1'].isnull(), 
                                 scores_red['winner_name'], 
                                 scores_red['loser_name'])
scores_red['player1'] = np.where(scores_red['player1'].isnull(), 
                                 scores_red['loser_name'], 
                                 scores_red['player1'])
scores_red['lose'] = np.where(scores_red['win'] == 1, 0, 1)
scores_red['h2h_win'] = scores_red[['player1', 'player2', 'win']].groupby(['player1', 'player2']).cumsum()
scores_red['h2h_lose'] = scores_red[['player1', 'player2', 'lose']].groupby(['player1', 'player2']).cumsum()
scores_red['h2h_win_lag'] = scores_red.groupby(['player1', 'player2'])['h2h_win'].shift(1)
scores_red['h2h_lose_lag'] = scores_red.groupby(['player1', 'player2'])['h2h_lose'].shift(1)
scores_red['year'] = scores_red['year'].astype('int')
                        
# filtering and final processing                        
h2h_fin = scores_red.loc[(scores_red.h2h_win_lag.notnull()) & (scores_red.year >= 1994)].reset_index(drop=True)
h2h_fin.drop(columns=['win', 'lose', 'h2h_win', 'h2h_lose', 'winner_name', 'loser_name'], inplace=True)
h2h_fin.rename(columns={'h2h_win_lag':'h2h_win','h2h_lose_lag':'h2h_lose'}, inplace=True)
h2h_fin['h2h_win'] = h2h_fin['h2h_win'].astype('int')
h2h_fin['h2h_lose'] = h2h_fin['h2h_lose'].astype('int')
h2h_fin['h2h_times'] = h2h_fin['h2h_win'] + h2h_fin['h2h_lose']

# write to df 
h2h_fin.to_csv("h2h_df.csv")
full_data.to_csv("player_df.csv")                       