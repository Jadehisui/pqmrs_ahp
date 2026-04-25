import numpy as np

BASE_WEIGHTS = np.array([0.4442, 0.2182, 0.1313, 0.0393, 0.0835, 0.0835])

def perturb_weights(weights: np.ndarray, sigma: float = 0.02) -> np.ndarray:
    candidate = weights + np.random.normal(0, sigma, len(weights))
    candidate = np.clip(candidate, 1e-6, None)
    return candidate / candidate.sum()

if __name__ == "__main__":
    np.random.seed(42)
    for i in range(10):
        w = perturb_weights(BASE_WEIGHTS)
        print(f"run={i:02d} weights={np.round(w, 4)}")
