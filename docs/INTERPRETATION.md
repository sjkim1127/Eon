# Structure & Useful God Specifications (Interpretation Logic)

Describes the core algorithmic framework for determining chart Structure (格局) and recommending the optimal Useful God (用神).

## 1. Structure (格局) Determination Algorithm

Evaluates the emergence of Month Branch hidden stems into the Heavenly Stems:

1. **Jianlu / Yang-in Priority**: When the Day Master attains seasonal authority matching Jianlu or Diwang (Yang-in), assigned as priority structure.
2. **Hidden Stem Manifestation (透出)**: Checks whether hidden stems of the month branch emerge in Year, Month, or Hour stems.
   - Primary (Main) stem manifested: Highest priority standard structure.
   - Middle / Remnant stem manifested: Secondary candidate when the main stem does not emerge.
3. **Month Command Default**: If no hidden stems emerge in the stems, the primary (main) stem of the month branch serves as the default structure.

## 2. Useful God (用神) Selection Process

### Eokbu Yongshin (Balance of Strength)

- **Weak Day Master**: Selects Resource (印星) or Parallel (比劫) elements to nourish and support the Day Master.
- **Strong Day Master**: Selects Output (食傷), Wealth (財星), or Officer (官星) elements to circulate, drain, or discipline excessive vigor.

### JoHu Yongshin (Climate Adjustment)

- **Winter Charts (Hai, Zi, Chou)**: **Fire (火)** is prioritized to warm cold, frozen energy pathways.
- **Summer Charts (Si, Wu, Wei)**: **Water (水)** is prioritized to cool arid, scorching temperatures.

### Primary Useful God (Yongshin) Finalization

1. When seasonal climate disparity is severe (Winter/Summer), JoHu takes precedence as the primary recommendation.
2. In moderate or balanced climates, Eokbu serves as the primary recommendation.
3. Favorable God (Heeshin / Assistant) is designated as the element generating or safeguarding the primary Useful God.

## 3. Spiritual Stars & Auxiliary Influences

The engine computes 20+ classical stars (Tianyi Noble, Munchang Noble, Kuigang, Hwagae, Yima, etc.), dynamically managed via the `spirit_markers.rs` module.
