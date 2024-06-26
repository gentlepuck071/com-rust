# Module Basics

In this tutorial, we'll explore how to use the `commune` library for module management in Python. The `commune` library provides functionalities for managing and serving code modules easily.

## Table of Contents
- [Finding Your Module](#finding-your-module)
- [Module Management](#module-management)
- [Serving](#serving)

---

## Finding Your Module

You can use the following steps to find and work with modules using the `commune` library.

### List All Modules
You can list all available modules using the following code:

```python
import commune as c

modules_list = c.modules()[:10]
c.print(modules_list)
```

### Searching for a Specific Module
To search for a specific module, you can use the `c.modules()` function with a search query:

```python
search_queries = ['model.llama', 'data', 'demo', 'hf']
for query in search_queries:
    c.print(f'Searching for {query}')
    c.print(c.modules(query))
```

---

## Module Management

Once you've found your module, you can manage it using the following steps.

### Accessing a Module
You can access a module using the `c.module()` function:

```python
demo = c.module('demo')
c.print('## Code for demo module')
c.print(demo.code())
```

### Viewing Module Config
You can view the configuration of a module using the `config()` method:

```python
demo.config()
```

### Listing Module Functions
To list the functions of a module, use the `fns()` method:

```python
demo_functions = demo.fns()
c.print(demo_functions)
```

### Searching for a Function
To search for a specific function within a module, use the `fns()` method with a search query:

```python
function_search_query = 'bro'
matching_functions = demo.fns(function_search_query)
c.print(matching_functions)
```

### Getting Function Schema
You can retrieve the schema of a specific function using the `schema()` method:

```python
function_name = 'bro'
function_schema = demo.schema(function_name)
c.print(function_schema)
```
---

## Serving

You can serve a module to make its functions accessible via a server.

### Serving a Module
You can serve a module using the `serve()` method, optionally providing a tag for versioning:

```python
demo.serve(tag='tag1')
```

### Viewing Available Servers
You can view the available servers using the `servers()` method:

```python
c.print(c.servers())
```

### Viewing Server Logs
To view the logs of a served module, you can use the `logs()` method:

```python
logs = c.logs('demo::tag1', mode='local')
c.print(logs)
```

### Connecting to a Served Module
You can connect to a served module using the `connect()` method:

```python
demo_client = c.connect('demo::tag1')
demo_client.info()
```

### Restarting and Killing a Served Module
You can restart or kill a served module using the `restart()` and `kill()` methods:

```python
c.restart('demo::tag1')  # Restart the module
c.kill('demo::tag1')     # Kill the module
```

---


This concludes our tutorial on module management using the `commune` library. You've learned how to find modules, manage their functions, serve them, and interact with served modules. This library can greatly simplify the process of managing and deploying code modules in your projects.
```

Feel free to use and adapt this markdown document for your tutorial needs. Make sure to adjust any details as necessary and include code snippets or explanations for each step to ensure clarity and comprehensiveness.