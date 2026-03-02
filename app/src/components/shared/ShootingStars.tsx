import { useMemo } from "react";

/** 배경 유성 애니메이션 */
export function ShootingStars() {
  const stars = useMemo(
    () =>
      [...Array(5)].map(() => ({
        top: `${Math.random() * 70}%`,
        left: `${Math.random() * 70}%`,
        delay: `${Math.random() * 10}s`,
        duration: `${3 + Math.random() * 3}s`,
      })),
    []
  );

  return (
    <div className="fixed inset-0 pointer-events-none overflow-hidden z-0">
      {stars.map((star, i) => (
        <div
          key={i}
          className="shooting-star animate-shot-star"
          style={{
            top: star.top,
            left: star.left,
            animationDelay: star.delay,
            animationDuration: star.duration,
          }}
        />
      ))}
    </div>
  );
}
