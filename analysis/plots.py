import pandas as pd
import matplotlib.pyplot as plt


def plot_rankings(input_csv: str = "../data/chains.csv", output_png: str = "ranking_scores.png") -> None:
    df = pd.read_csv(input_csv)
    # Equal weights here for visualization only; Rust produces authoritative weighted scores.
    score_cols = ["crypto_flex", "governance", "runtime", "throughput", "wallet", "validator"]
    df["mean_score"] = df[score_cols].mean(axis=1)
    df = df.sort_values("mean_score", ascending=False)

    plt.figure(figsize=(8, 4.5))
    plt.bar(df["chain"], df["mean_score"], color=["#1f77b4", "#ff7f0e", "#2ca02c", "#d62728", "#9467bd"])
    plt.title("Chain Baseline Scores (Unweighted Mean)")
    plt.ylabel("Score")
    plt.tight_layout()
    plt.savefig(output_png, dpi=200)
    print(f"saved {output_png}")


if __name__ == "__main__":
    plot_rankings()
