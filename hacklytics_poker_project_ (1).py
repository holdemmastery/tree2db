# -*- coding: utf-8 -*-
"""Hacklytics Poker Project .ipynb

Automatically generated by Colaboratory.

Original file is located at
    https://colab.research.google.com/drive/1CedYq8Nlr-nAIXVWKDFnkDkzgUJzxEE_
"""

import numpy as np
import pandas as pd
import random
from scipy.stats import norm
from scipy.stats import uniform
import requests
import json
import matplotlib.pyplot as plt

ELO_array = np.zeros(100)
for player in range(len(ev_array)):
  ELO_array[player] = (1.2*np.sum(ev_array[player][:2000]/2000)) + (1.1 * np.sum(ev_array[player][2001:3000]/1000))+  (np.sum(ev_array[player][3001:4000]/1000))
ELO_array += 100
ELO_array.min(), ELO_array.max()

ranks = ['2', '3', '4', '5', '6', '7', '8', '9', '10', 'J', 'Q', 'K', 'A']
suits = ['Hearts', 'Diamonds', 'Clubs', 'Spades']
hole_cards = {}
card_number = 1
for rank in range(len(ranks)):
    for suit in range(len(suits)):
      hole_cards[card_number] = str(ranks[rank]) + " of " + str(suits[suit])
      card_number += 1
list_hole_cards = list(hole_cards.values())
positions = ["BTN","SB","BB","UTG","HJ","CO"]
dealt_hole_cards = random.sample(list_hole_cards,2)
position = random.sample(positions, 1)[0]
print("You are dealt", dealt_hole_cards[0], dealt_hole_cards[1], "in the", position, "position.")

#Button decision making
if position == "BTN":
  if dealt_hole_cards[0][0] == dealt_hole_cards[1][0]:
    print("You should raise this hand to 2.5 BB.")

import numpy as np

def update_ev(current_rating, ev_loss, games_played):
    K_BASE = 20  # Base K-factor for rating adjustments
    K = K_BASE * (1 / 100* np.log(0.4 * (games_played + 6))) * (1 / 100 * np.log(0.4 * (current_rating + 6)))

    # Calculate rating change based on EV loss
    if ev_loss == 0:
        rating_change = K
    else:
        # Decrease rating based on the severity of EV loss, assuming EV loss is properly scaled
        rating_change = K * (ev_loss*0.125)  # Assuming EV loss is scaled; adjust divisor as needed for actual scale
        # print(rating_change)
    if games_played < 250:
      rating_change *= 800
    elif games_played < 500:
      rating_change *= 400
    elif games_played < 750:
      rating_change *= 200
    elif games_played < 1000:
      rating_change *= 100
    newrating = current_rating + rating_change
    return newrating

#random seed 1 and 27
np.random.seed(1)
current_ratinglist = []
ev_array = np.zeros(1000)
for i in range(0,len(ev_array),2):
  ev_array[i] = -1 * np.random.exponential(7.5)
ev_array.min(),ev_array.max()
current_rating = 1000
for i in range(len(ev_array)):
  current_rating = (update_ev(current_rating, ev_array[i], i))
  if i % 10 == 0 :
    current_ratinglist.append(current_rating)
print(current_ratinglist)

class Problem:
    def __init__(self, time_created: int, hero_position: Position, villain_position: Position,
                 hole_cards: str, board: str, action_history: List[Action], river_option_evs: List[OptionEVPair]):
        self.time_created = time_created
        self.hero_position = hero_position
        self.villain_position = villain_position
        self.hole_cards = hole_cards
        self.board = board
        self.action_history = action_history
        self.river_option_evs = river_option_evs

plt.plot(current_ratinglist)

plt.plot(ev_array[:100])

