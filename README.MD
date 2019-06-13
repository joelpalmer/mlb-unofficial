 **Initial Requirements:**
 
- [x]  Take in arguments for `TEAM` & `DATE` (default today) -> score & maybe opponent...
    * ~~Trying~~ using clap 
- [x]  Possible states: no game, delayed, postponed, on later, in progress, final
- [ ]  Data Source: https://statsapi.mlb.com/api/v1/schedule?sportId=1,51&date=2019-06-10
- [ ]  todo: add steps like json parsing etc...
- [ ]  error out on wrong date format or attempt to format
- [ ]  Tests! 
- [ ]  String formats & messages for different game states 
- [ ]  Bring in termcolor or similar to use team colors? (Do I really want to maintain a team color list even though it is pretty static?). What else could I do with colors in the term?

`mlb dodgers 2019-6-20`
> `Dodgers are beating the Giants: 22 to 0`