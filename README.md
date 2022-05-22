# tfc-which-workspace

## What is this?

This tool is for searching for workspaces in Terraform Cloud. It first attempts to pull a list of workspaces for the specified organization, it can optionally accept a name parameter to filter the initial list. After the list of workspaces has been retrieved the tool can then attempt to further filter the list based on the provided query parameters.

Currently the only provided filter is for variable based filtering. If one or more variable filters have been provided the tool will attempt to work through the passed filter rules. For each workspace in the list an additional call will be made to pull the variables for that workspace. After that data has been gathered it will then run through the variable filter logic. See [Variable Filters](#variable-filters) for more details.

After the filter logic has completed the resulting dataset will be outputted both to the terminal and to a file. By default the file name will be `result.json` and it will be placed in the working directory in which the tool was run.

Settings can be provided by either providing a `settings.toml` file or by passing ENV variables along before the command. Caching is leveraged on all remote calls (following http caching rules) and will create a directory in the working directory in which the tool was run named `http-cacache` where the cache files will reside.

## Why tho?

Because I didn't want to do it manually and I felt like it

## Variable Filters

First let's take a look at an example variable filter setup.

```toml
...
[query] # Required but can be left empty
name = "aws-" # Optional

[[query.variables]] # Optional
key = "mode" # Required
operator = "Contains" # Required
value = "prod" # Required

[[query.variables]] # Optional
key = "status" # Required
operator = "NotEqual" # Required
value = "migrating" # Required
...
```

In this example we will first have an initial name filter, looking only for workspaces with a name starting with `aws-`. We then add two variable filters to our query. The first filter will require that the workspace has a variable with a key of `mode` and a value containing the string `prod`. The second filter will check the variable with the key of `status`, should it exist, and verify that it does not exactly equal `migrating`. So our resulting dataset would contain only those workspaces starting the with the name `aws-`, containing the string `prod` in the `mode` key, and will not have a `status` of `migrating` should the key exist.

### Variable Filters - Operators

Currently the available "operators" are:

- Equals
  - A variable with the specified key must exist, and must exactly equal the specified value
- NotEquals
  - Should a variable with the specified key exist it must not exactly equal the specified value
- Contains
  - A variable with the specified key must exist, and must contain the specified value
- NotContains
  - Should a variable with the specified key exist it must not contain the specified value
  
## Notes

Rate limiting hasn't been implemented yet so be aware, see [Terraform Docs](https://www.terraform.io/cloud-docs/api-docs#rate-limiting) for information on those limits. Pagination has been implemented but take special care of setting `max_depth` = `0` to pull all pages as this could result in a large number of calls. There are also probably some bugs so use at your own risk!
