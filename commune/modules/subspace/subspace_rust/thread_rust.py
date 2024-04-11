from concurrent.futures import ThreadPoolExecutor
import rust_thread_executor # type: ignore

def sum_fn(start, end):
    sum_value = 0
    for i in range(start, end):
        sum_value += i
    print(sum_value)

executor = ThreadPoolExecutor(max_workers=2)
future_1 = executor.submit(
    ThreadPoolExecutor.execute_python_function_allow_threads(sum_fn, (1, 10000001))
)
future_2 = executor.submit(
    rust_thread_executor.execute_python_function_allow_threads(sum_fn, (1, 10000001))
)