#!/usr/bin/env python3
"""
update_brasileirao.py
Automated tool to download and update the official Brasileirão Série A
standings (scoreboard) and remaining matches directly from open APIs.

This script:
1. Downloads the current standings (points, wins, games played) for all 20 teams.
2. Downloads the official schedule of remaining matches (matches not yet finished),
   preserving the exact chronological order and date set by CBF.
3. Normalizes team names to canonical forms used by the Rust simulator.
4. Validates data integrity before writing to disk.

No third-party packages required (pure Python standard library).
"""

import sys
import os
import json
import argparse
from datetime import datetime
from urllib.request import Request, urlopen
from urllib.error import URLError

ESPN_STANDINGS_URL = "https://site.api.espn.com/apis/v2/sports/soccer/bra.1/standings"
ESPN_SCOREBOARD_URL = "https://site.api.espn.com/apis/site/v2/sports/soccer/bra.1/scoreboard"

NAME_MAPPING = {
    "Athletico Paranaense": "Athletico-PR",
    "Atlético Paranaense": "Athletico-PR",
    "Athletico-PR": "Athletico-PR",
    "Atlético Mineiro": "Atlético-MG",
    "Atlético-MG": "Atlético-MG",
    "Atletico-MG": "Atlético-MG",
    "Atletico MG": "Atlético-MG",
    "Red Bull Bragantino": "Red Bull Bragantino",
    "Bragantino": "Red Bull Bragantino",
    "RB Bragantino": "Red Bull Bragantino",
    "Vasco da Gama": "Vasco da Gama",
    "Vasco": "Vasco da Gama",
    "São Paulo": "São Paulo",
    "Sao Paulo": "São Paulo",
    "Grêmio": "Grêmio",
    "Gremio": "Grêmio",
    "Vitória": "Vitória",
    "Vitoria": "Vitória",
    "Chapecoense": "Chapecoense",
    "Chapecoense-SC": "Chapecoense",
    "Remo": "Remo",
    "Clube do Remo": "Remo",
    "Coritiba": "Coritiba",
    "Coritiba F.C.": "Coritiba",
    "Mirassol": "Mirassol",
    "Mirassol-SP": "Mirassol",
}

def normalize_name(name: str) -> str:
    cleaned = name.strip()
    return NAME_MAPPING.get(cleaned, cleaned)

def fetch_standings():
    """Fetch current official league table from ESPN."""
    print("Downloading updated official standings...")
    req = Request(ESPN_STANDINGS_URL, headers={"User-Agent": "Mozilla/5.0"})
    with urlopen(req, timeout=15) as resp:
        data = json.loads(resp.read().decode("utf-8"))

    entries = data["children"][0]["standings"]["entries"]
    teams = []
    for entry in entries:
        raw_name = entry["team"]["displayName"]
        name = normalize_name(raw_name)
        stats = {}
        for s in entry.get("stats", []):
            if "name" in s and "value" in s:
                stats[s["name"]] = s["value"]
        points = int(stats.get("points", 0))
        wins = int(stats.get("wins", 0))
        games = int(stats.get("gamesPlayed", 0))
        rank = int(stats.get("rank", len(teams) + 1))
        teams.append((rank, name, points, wins, games))

    teams.sort(key=lambda t: t[0])
    return [(name, pts, w, g) for _, name, pts, w, g in teams]

def fetch_remaining_matches(from_date: str = None):
    """
    Fetch all remaining scheduled matches from the official calendar.
    Matches already played (status != 'pre') are excluded.
    """
    if from_date is None:
        from_date = datetime.now().strftime("%Y-%m-%d")

    print(f"Fetching official match calendar from {from_date} onwards...")
    req = Request(ESPN_SCOREBOARD_URL, headers={"User-Agent": "Mozilla/5.0"})
    with urlopen(req, timeout=15) as resp:
        data = json.loads(resp.read().decode("utf-8"))

    calendar = data["leagues"][0]["calendar"]
    future_dates = [d[:10].replace("-", "") for d in calendar if d[:10] >= from_date]
    print(f"Found {len(future_dates)} dates with remaining rounds.")

    matches = []
    for dt_str in future_dates:
        url = f"{ESPN_SCOREBOARD_URL}?dates={dt_str}"
        req = Request(url, headers={"User-Agent": "Mozilla/5.0"})
        try:
            with urlopen(req, timeout=15) as resp:
                day_data = json.loads(resp.read().decode("utf-8"))
        except Exception as e:
            print(f"Error fetching matches for date {dt_str}: {e}")
            continue

        events = day_data.get("events", [])
        for ev in events:
            # Only consider scheduled or in-progress matches
            state = ev.get("status", {}).get("type", {}).get("state", "")
            if state == "post":
                continue

            date_raw = ev.get("date", "")
            if date_raw:
                try:
                    dt_obj = datetime.strptime(date_raw[:10], "%Y-%m-%d")
                    formatted_date = dt_obj.strftime("%d/%m")
                except ValueError:
                    formatted_date = f"{dt_str[6:8]}/{dt_str[4:6]}"
            else:
                formatted_date = f"{dt_str[6:8]}/{dt_str[4:6]}"

            competitions = ev.get("competitions", [{}])[0]
            competitors = competitions.get("competitors", [])
            home_team = None
            away_team = None
            for c in competitors:
                raw = c.get("team", {}).get("displayName", "")
                norm = normalize_name(raw)
                if c.get("homeAway") == "home":
                    home_team = norm
                else:
                    away_team = norm

            if home_team and away_team:
                matches.append((formatted_date, home_team, away_team))

    print(f"Total of {len(matches)} remaining matches to simulate.")
    return matches

def validate_data(teams, matches):
    """Validate consistency of teams and matches."""
    team_names = {name for name, _, _, _ in teams}
    if len(team_names) != 20:
        raise ValueError(f"Expected 20 teams in the standings, found {len(team_names)}: {team_names}")

    for name, pts, wins, games in teams:
        max_possible = wins * 3 + (games - wins)
        if pts > max_possible:
            raise ValueError(f"Point inconsistency for {name}: {pts} pts in {games} games ({wins} wins). Maximum possible: {max_possible}")

    for idx, (dt, home, away) in enumerate(matches, 1):
        if home not in team_names:
            raise ValueError(f"Match {idx} ({dt}): Home team '{home}' is not in the teams table!")
        if away not in team_names:
            raise ValueError(f"Match {idx} ({dt}): Away team '{away}' is not in the teams table!")

    return True

def write_files(teams, matches, teams_file: str, matches_file: str):
    """Write teams and matches to semicolon-delimited files."""
    with open(teams_file, "w", encoding="utf-8") as f:
        for name, pts, wins, games in teams:
            f.write(f"{name};{pts};{wins};{games}\n")
    print(f"Standings file successfully saved: {teams_file}")

    with open(matches_file, "w", encoding="utf-8") as f:
        for dt, home, away in matches:
            f.write(f"{dt};{home};{away}\n")
    print(f"Remaining matches file successfully saved: {matches_file}")

def main():
    parser = argparse.ArgumentParser(description="Brasileirão Standings and Matches Updater")
    parser.add_argument("--teams", default="times2026.txt", help="Path to teams standings file (default: times2026.txt)")
    parser.add_argument("--matches", default="jogos2026.txt", help="Path to remaining matches file (default: jogos2026.txt)")
    parser.add_argument("--from-date", default=None, help="Cutoff date YYYY-MM-DD for upcoming matches (default: current date)")
    args = parser.parse_args()

    teams = fetch_standings()
    matches = fetch_remaining_matches(args.from_date)
    validate_data(teams, matches)
    write_files(teams, matches, args.teams, args.matches)
    print("Update completed successfully!")

if __name__ == "__main__":
    main()
