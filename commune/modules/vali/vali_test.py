import rust_vali_thread

def sum_fn(start, end):
    sum_value = 0
    for i in range(start, end):
        sum_value += i
    print(sum_value)

A = rust_vali_thread.create_thread(sum_fn, (1, 101))
print("here => ", A)