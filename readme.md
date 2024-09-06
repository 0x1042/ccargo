
> generate cmake project 

**usage**

```
 ./ccargo --help
Usage: ccargo [OPTIONS] --name <NAME>

Options:
  -n, --name <NAME>      project name
      --cstd <CSTD>      c stanard [default: 11]
      --cppstd <CPPSTD>  cpp stanard [default: 20]
      --asan             generate option for address sanitize
      --tsan             generate option for thread sanitize
      --memory           generate option for memory sanitize
      --undefined        generate option for undefined sanitize
      --tidy             generate option for clang-tidy
      --dwarf            generate option for split-dwarf
  -h, --help             Print help
```