# from substrate_subxt_module import create_substrate_interface

# params = {
#     "url": "wss://your-substrate-node-url",
#     "websocket": "wss://your-websocket-url",
#     "ss58_format": 42,
#     "type_registry": "your-type-registry",
#     "type_registry_preset": "your-type-registry-preset",
#     "cache_region": "your-cache-region",
#     "runtime_config": "your-runtime-config",
#     "ws_options": "your-ws-options",
#     "auto_discover": True,
#     "auto_reconnect": True,
# }

# create_substrate_interface(params)

import substrate_subxt_module # type: ignore
import time

start_time = time.time()
sub = substrate_subxt_module.create_substrate_interface()
print("substrate:","network?", sub)
end_time = time.time()

running_time = end_time - start_time
print(f"The code took {running_time:.4f} seconds to run.")
