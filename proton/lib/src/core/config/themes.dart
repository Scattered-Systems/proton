part of 'settings.dart';

enum Themes implements Comparable<Themes> {
  light,
  dark;

  static Themes fromIndex(int index) {
    return Themes.values[index];
  }

  static Themes fromBrightness(Brightness brightness) {
    switch (brightness) {
      case Brightness.light:
        return Themes.light;
      case Brightness.dark:
        return Themes.dark;
    }
  }

  @override
  int compareTo(Themes other) {
    return index.compareTo(other.index);
  }

  Brightness get brightness {
    switch (this) {
      case Themes.light:
        return Brightness.light;
      case Themes.dark:
        return Brightness.dark;
    }
  }

  ThemeMode get mode {
    switch (this) {
      case Themes.light:
        return ThemeMode.light;
      case Themes.dark:
        return ThemeMode.dark;
    }
  }

  ButtonBarThemeData get buttonBarTheme {
    return const ButtonBarThemeData(
      alignment: MainAxisAlignment.spaceEvenly,
      buttonPadding: EdgeInsets.all(16),
      buttonTextTheme: ButtonTextTheme.accent,
      mainAxisSize: MainAxisSize.max,
    );
  }

  BottomNavigationBarThemeData get bottomNavigationBarTheme {
    return BottomNavigationBarThemeData(
      backgroundColor: Colors.transparent,
      selectedIconTheme: selectedIconTheme,
      selectedItemColor: primarySwatch[800],
      unselectedItemColor: primarySwatch[300],
      unselectedLabelStyle: TextStyle(color: primarySwatch[300]),
    );
  }

  DrawerThemeData get drawerTheme {
    return const DrawerThemeData(
      elevation: 0,
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.only(
          topRight: Radius.circular(16),
          bottomRight: Radius.circular(16),
        ),
      ),
    );
  }

  FloatingActionButtonThemeData get floatingActionButtonTheme {
    return FloatingActionButtonThemeData(
      backgroundColor: primarySwatch[800],
      foregroundColor: primarySwatch[50],
    );
  }

  IconThemeData get iconTheme {
    return IconThemeData(
      color: primaryColor,
    );
  }

  IconThemeData get selectedIconTheme {
    return IconThemeData(
      color: primarySwatch[900],
    );
  }

  ThemeData get theme {
    return ThemeData(
        bottomNavigationBarTheme: bottomNavigationBarTheme,
        brightness: brightness,
        buttonBarTheme: buttonBarTheme,
        cardColor: cardColor,
        drawerTheme: drawerTheme,
        iconTheme: iconTheme,
        primaryColor: primaryColor,
        primaryColorDark: primaryColorDark,
        primaryColorLight: primaryColorLight,
        primarySwatch: primarySwatch,
        useMaterial3: true);
  }
}


extension ColorExtension on Themes {
  MaterialAccentColor get accent {
    switch (this) {
      case Themes.dark:
        return Colors.purpleAccent;
      case Themes.light:
        return Colors.deepPurpleAccent;
    }
  }

  Color get cardColor {
    switch (this) {
      default:
        return secondarySwatch[700]!;
    }
  }

  Color get hoverColor {
    return primarySwatch[800]!;
  }

  Color get primaryColor {
    switch (this) {
      case Themes.light:
        return Colors.purple;
      case Themes.dark:
        return Colors.deepPurple;
    }
  }

  Color get primaryColorDark {
    switch (this) {
      case Themes.dark:
        return Colors.white;
      default:
        return Colors.black;
    }
  }

  Color get primaryColorLight {
    switch (this) {
      case Themes.dark:
        return Colors.black;
      default:
        return Colors.white;
    }
  }

  MaterialColor get primarySwatch {
    switch (this) {
      case Themes.dark:
        return Colors.purple;
      case Themes.light:
        return Colors.deepPurple;
    }
  }

  MaterialColor get secondarySwatch {
    switch (this) {
      case Themes.dark:
        return Colors.purple;
      default:
        return Colors.deepPurple;
    }
  }
}
