export 'pages/home.dart' show HomeScreen;

import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:proton/proton.dart' show TabEnumMixin;

enum AppScreens with TabEnumMixin implements Comparable<AppScreens> {
  home(),
  feed(),
  maps(),
  settings();

  static AppScreens fromIndex(int index) {
    return values[index];
  }

  static int locationToIndex(String location) {
    final index = values.indexWhere((t) => location.contains(t.name));
    // if index not found (-1), return 0
    return index < 0 ? 0 : index;
  }

  static BottomNavigationBar bottomNavigationBar(BuildContext context) {
    return BottomNavigationBar(
      items: values.map((t) => t.bottomNavItem).toList(),
      currentIndex: locationToIndex(GoRouter.of(context).location),
      onTap: (index) {
        GoRouter.of(context).go(values[index].path);
      },
    );
  }

  @override
  int compareTo(AppScreens other) {
    return index.compareTo(other.index);
  }

  NavigationDestination get dest {
    return NavigationDestination(icon: Icon(icon), label: label);
  }

  BottomNavigationBarItem get bottomNavItem {
    return BottomNavigationBarItem(icon: Icon(icon), label: label);
  }

  ListTile tile(BuildContext context) {
    return ListTile(
      onTap: () {
        GoRouter.of(context).pop();
        GoRouter.of(context).go(path);
      },
      title: Text(label),
      leading: Icon(icon),
    );
  }

  String get path {
    switch (this) {
      case AppScreens.home:
        return '/';
      default:
        return '/$name';
    }
  }

  IconData get icon {
    switch (this) {
      case AppScreens.feed:
        return Icons.rss_feed;
      case AppScreens.home:
        return Icons.home;
      case AppScreens.maps:
        return Icons.map_outlined;
      case AppScreens.settings:
        return Icons.settings;
    }
  }
}
