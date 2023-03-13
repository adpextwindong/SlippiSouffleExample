# Slippi Souffle Example

An expeirment with analyzing a Super Smash Bros. Melee replay file from Slippi using Souffle Datalog.

## Building and Running

```
nix-shell
cargo run && souffle example.dl && cat UpSmash.csv
```

## The Datalog

```datalog

.decl prestate(player : number, frame: number, state: number)
.input prestate

.decl poststate(player : number, frame: number, state: number)
.input poststate

.decl Trans(player :number, s1 : number, s2 :number, ts :number)

//Player Action State transition
Trans(player, s1, s2, ts):-
  prestate(player, ts, s1),
  poststate(player, ts, s2).

.output Trans

.decl UpSmash(player : number, ts : number)

UpSmash(player, ts) :-
  Trans(player, s1, 63, ts),
  s1 != 63.

//https://github.com/hohav/peppi/blob/main/peppi/src/model/enums/action_state.rs#L174

.output UpSmash
```

Output for `singleUpsmash.slp`

```
1	93
```

Read as player 1 performed an UpSmash on frame 93 (After game entry is over).
