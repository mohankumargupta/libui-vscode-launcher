# Rust binding for libui-ng

Already having issues: 

To install VS Build Tools 2022 using

```sh
winget install Microsoft.VisualStudio.2022.BuildTools -e --source winget --silent --override "--wait --quiet --add ProductLang En-us --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
```

variation

```sh
winget install --force Microsoft.VisualStudio.2022.BuildTools -e --source winget --override "--wait --add ProductLang En-us --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
```

```sh
winget install Microsoft.VisualStudio.2022.Community --silent --override "--wait --quiet --add ProductLang En-us --add Microsoft.VisualStudio.Workload.NativeDesktop --includeRecommended"
```

Installing LLVM

```sh
winget install -i LLVM.LLVM
```
