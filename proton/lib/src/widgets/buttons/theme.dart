import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

/// A [ChangeNotifier] that holds the current [ThemeMode].
class ThemeModeNotifier extends ChangeNotifier {
  ThemeMode _themeMode = ThemeMode.system;

  ThemeMode get themeMode => _themeMode;

  void setThemeMode(ThemeMode mode) {
    _themeMode = mode;
    notifyListeners();
  }

  void toggleThemeMode(BuildContext context) {
    if (_themeMode == ThemeMode.light) {
      _themeMode = ThemeMode.dark;
    } else if (_themeMode == ThemeMode.dark) {
      _themeMode = ThemeMode.light;
    } else {
      final Brightness platformBrightness =
          MediaQuery.of(context).platformBrightness;
      if (platformBrightness == Brightness.dark) {
        _themeMode = ThemeMode.light;
      } else {
        _themeMode = ThemeMode.dark;
      }
    }
    notifyListeners();
  }
}

/// [ThemeToggle] works with [ThemeModeNotifier] to toggle the [ThemeMode] of
/// the app.
class ThemeToggle extends StatelessWidget {
  const ThemeToggle({Key? key, this.tooltip = 'Toggle the theme'}) : super(key: key);
  final String tooltip;

  @override
  Widget build(BuildContext context) {
    final themeModeNotifier = context.watch<ThemeModeNotifier>();
    return IconButton(
      icon: Icon(icon(themeModeNotifier.themeMode)),
      onPressed: () {
        themeModeNotifier.toggleThemeMode(context);
      },
      tooltip: tooltip,
    );
  }

  IconData icon(ThemeMode mode) {
    if (mode == ThemeMode.light) {
      return Icons.dark_mode;
    }
    return Icons.light_mode;
  }
}
