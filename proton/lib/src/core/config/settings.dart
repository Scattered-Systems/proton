import 'package:flutter/material.dart';

part 'appkeys.dart';
part 'themes.dart';

class AppSettings {
  final String title;

  const AppSettings({this.title = 'Proton'});


  ThemeData get darkTheme => Themes.dark.theme;
  ThemeData get theme => Themes.light.theme;
}
