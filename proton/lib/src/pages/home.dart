import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';

class HomeScreen extends StatefulWidget {
  final String label;

  const HomeScreen({Key? key, this.label = 'Home'}) : super(key: key);

  Page<HomeScreen> pageBuilder(
      BuildContext context, GoRouterState state) {
    return MaterialPage<HomeScreen>(
      name: 'home',
      key: const ValueKey<String>('home'),
      child: this,
    );
  }

  static String get path => '/';

  static GoRoute route(
      {String label = 'Home', GlobalKey<NavigatorState>? parentNavigatorKey}) {
    HomeScreen home = HomeScreen(label: label);
    return GoRoute(
        name: home.label.toLowerCase(),
        path: path,
        pageBuilder: home.pageBuilder,
        parentNavigatorKey: parentNavigatorKey
      );
  }

  @override
  State<HomeScreen> createState() => _HomeScreenState();
}

class _HomeScreenState extends State<HomeScreen> {
  @override
  Widget build(BuildContext context) {
    return const Scaffold(
      body: Center(
        child: Text('Home Screen'),
      ),
    );
  }
}
