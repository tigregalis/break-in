# Breakin

```
git clone https://github.com/tigregalis/break-in.git
cd break-in
cargo run --release
```

Like breakout, but in.

Two modes:

- Classic: destroy all destructible bricks without dying; some bricks are worth more
- Golf: get the ball into the centre with the least number of hits

Variants:

- Some bricks are indestructible
- Some bricks "regrow" (hits left) if not destroyed
- Bricks orbit around the centre
- You get one Save (constructs a ring behind the paddle which lasts 200ms), but it halves your score (in Classic) or doubles your score (in Golf)
