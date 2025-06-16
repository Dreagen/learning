# OpenCode.md

## Build, Lint, and Test Commands
- Build command: Not explicitly defined; Flutter projects are typically built with `flutter build`.
- Lint command: Ensure linting by running `flutter analyze`.
- Test command: Run all tests using `flutter test`.
- Run a single test: Execute `flutter test test/widget_test.dart` to run only the widget test.

## Code Style Guidelines
1. **Imports**:
   - Use organized imports such as: material components (`package:flutter/material.dart`) and test utilities (`package:flutter_test/flutter_test.dart`).
   - Maintain separate lines for each package.
2. **Formatting**:
   - Follow Flutter formatting standards. Run `dart format .` to auto-format.
4. **Types**:
   - Use Dart's strong typing, including type annotations where possible.
5. **Naming Conventions**:
   - Name variables in camelCase.
   - Capitalize class names in PascalCase.
   - Use descriptive names for methods, classes, and variables.
6. **Error Handling**:
   - Include explicit error handling and avoid silent failures.
7. **Flutter-specific Best Practices**:
   - Use `testWidgets` API for widget tests.
   - Prefer use of const constructors where possible.

## Lints
Defined in `analysis_options.yaml`:
- Do not force `prefer_const_constructors_in_immutables`, `prefer_const_constructors`, or similar rules that restrict use of flexibility.
- Enable rules like `unnecessary_breaks` to cleanup Dart syntax.

For more elaborate setups and customization, consider directly inspecting `pubspec.yaml` and `analysis_options.yaml`!
