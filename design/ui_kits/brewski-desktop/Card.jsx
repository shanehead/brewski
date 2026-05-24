// Card — the panel wrapper used throughout the Brewski app.
// Title strip (UPPERCASE eyebrow) above a 1px divider, body padded 14px.

function Card({ title, children, style }) {
  return (
    <div className="card" style={style}>
      {title && (
        <div className="card-head"><span>{title}</span></div>
      )}
      <div className="card-body">{children}</div>
    </div>
  );
}

window.Card = Card;
