class Greeter:
    def __init__(self, rng):
        self.rng = rng

    def greet(self) -> string:
        if self.rng.next_u32(1) == 0:
            return "hello from python"
        else:
            return "bye bye from python"