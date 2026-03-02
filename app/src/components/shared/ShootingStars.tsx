/** 배경 유성 애니메이션 */
export function ShootingStars() {
  return (
    <div className="fixed inset-0 pointer-events-none overflow-hidden z-0">
      {[...Array(5)].map((_, i) => (
        <div
          key={i}
          className="shooting-star"
          style={{
            top: `${Math.random() * 50}%`,
            left: `${Math.random() * 50}%`,
            animationDelay: `${Math.random() * 10}s`,
          }}
        />
      ))}
    </div>
  );
}
