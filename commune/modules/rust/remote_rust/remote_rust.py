import c_random_color_module # type: ignore
import time

start_time = time.time()
color = c_random_color_module.c_random_color()
print("Random color:", color)
end_time = time.time()

running_time = end_time - start_time
print(f"The code took {running_time:.4f} seconds to run.")
