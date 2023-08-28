class SamplePage:
  def fib(self, n: int) -> int:
    if n <= 2:
      return 1
    else:
      return self.fib(n - 1) + self.fib(n - 2)

  def render(self) -> str:
    x = 12
    y = 5.5
    return f"<div>" \
           f"<h1 style='color: blue;'>Header</h1>" \
           f"<p style='color: red;'>Sample paragraph</p>" \
           f"<p>{x} + {y} = {x + y}</p>" \
           f"<p>{x} / {y} = {x / y}</p>" \
           f"<p>The 10th fibonacci number is {self.fib(10)}</p>" \
           f"</div>"


SamplePage()
