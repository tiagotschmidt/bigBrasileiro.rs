# bigBrasileiro.rs

A computational model written in **Rust** using **Monte Carlo simulation** to predict final probabilities for the Brazilian National Football Championship (Campeonato Brasileiro Série A).

The simulator calculates probabilities for each club:
- Becoming **Champion** (1st place)
- Becoming **Runner-up** (2nd place)
- Being **Relegated** (Z-4: 17th to 20th places)
- Full percentage distribution across all 20 table positions
- **Expected average points** for every placement at the end of the tournament (points threshold for champion, continental spots, and relegation survival)

---

## 🚀 How to Run

### 1. Running the Simulation

The simulator dynamically switches `MAX_SIM` depending on the environment:
- **GitHub Actions / CI (`true` or `GITHUB_ACTIONS=true`):** Uses `MAX_SIM_GITHUB = 100_000_000` (calibrated to complete in ~10 minutes on a 2-vCPU runner).
- **Local Deployment (`false` or default):** Uses `MAX_SIM_LOCAL = 1_000_000_000` (deep billion-iteration simulation for maximum statistical accuracy).
- **Custom Override:** You can pass a custom count as the second argument or via `MAX_SIM` environment variable.

```bash
# GitHub Actions mode (stdout output, 100M simulations):
cargo run --manifest-path brasileirao-simulator/Cargo.toml --release -- true

# Local deployment mode (saves reports to logs/, 1B simulations):
cargo run --manifest-path brasileirao-simulator/Cargo.toml --release -- false

# Custom iteration count (e.g., quick 100k run for testing):
cargo run --release -- true 100000

# Or via environment variable:
MAX_SIM=1000000 cargo run --release -- false
```

### 2. Running Unit Tests

```bash
cd brasileirao-simulator
cargo test
```

---

## 🔄 Automated Data Updates (Standings & Remaining Fixtures)

The repository includes the [`scripts/update_brasileirao.py`](scripts/update_brasileirao.py) script (standard Python, zero external pip dependencies) to automatically download and sync the latest standings and upcoming matches.

### Fetch current standings and remaining official CBF matches:
```bash
python3 scripts/update_brasileirao.py
```
This fetches:
1. The **current official table** (points, victories, games played for all 20 teams).
2. The **official remaining fixtures** in chronological order, automatically excluding matches that have already finished.

### Optional parameters:
- `--teams <path>`: Output path for the teams file (default: `times2026.txt`).
- `--matches <path>`: Output path for the remaining matches file (default: `jogos2026.txt`).
- `--from-date <YYYY-MM-DD>`: Cutoff date for upcoming matches (defaults to today's date).

Example:
```bash
python3 scripts/update_brasileirao.py --teams times2026.txt --matches jogos2026.txt
```

---

## 📁 Input File Format

1. **`times2026.txt`** (or `times<date>.txt`):
   ```text
   TeamName;Points;Wins;Games
   Flamengo;70;21;32
   Palmeiras;69;20;32
   ...
   ```
2. **`jogos2026.txt`** (or `jogos<date>.txt`):
   ```text
   Date;HomeTeam;AwayTeam
   28/10;Fluminense;Internacional
   28/10;Santos;Palmeiras
   ...
   ```

