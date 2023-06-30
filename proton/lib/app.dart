import 'dart:async';

import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:provider/provider.dart';

import 'proton.dart';

class Proton extends StatefulWidget {
  final AppSettings settings;

  const Proton({Key? key, required this.settings}) : super(key: key);

  @override
  State<Proton> createState() => _ProtonState();
}

class _ProtonState extends State<Proton> {
  // Keys
  final GlobalKey<NavigatorState> _rootNavKey = GlobalKey();
  final GlobalKey<ScaffoldMessengerState> _scaffoldMessengerKey = GlobalKey();
  final GlobalKey<NavigatorState> _shellNavKey = GlobalKey();

  ThemeMode _themeMode = ThemeMode.system;

  set setTheme(ThemeMode themeMode) {
    setState(() {
      _themeMode = themeMode;
    });
  }

  AppRouter get _router => AppRouter(settings: widget.settings);

  @override
  Widget build(BuildContext context) {
    final themeModeNotifier = context.watch<ThemeModeNotifier>();
    themeModeNotifier.addListener(() {
      if (themeModeNotifier.themeMode != _themeMode) {
        setTheme = themeModeNotifier.themeMode;
      }
    });

    return MaterialApp.router(
      darkTheme: widget.settings.darkTheme,
      routerConfig: _router(rootNavkey: _rootNavKey, shellNavkey: _shellNavKey),
      scaffoldMessengerKey: _scaffoldMessengerKey,
      theme: widget.settings.theme,
      title: widget.settings.title,
      themeMode: _themeMode,
    );
  }
}

/// [FluttersRouter] is the router for [Flutters]
/// It is responsible for:
/// - setting up the [GoRouter]
/// - setting up the [FluttersScaffold] for the [ShellRouter]
class AppRouter {
  final AppSettings settings;

  const AppRouter({
    required this.settings,
  });


  GoRouter call(
      {GlobalKey<NavigatorState>? rootNavkey,
      GlobalKey<NavigatorState>? shellNavkey}) {
    return GoRouter(
      initialLocation: '/',
      navigatorKey: rootNavkey,
      observers: [],
      redirect: _guard,
      routes: [
        ShellRoute(
            builder: (context, state, child) {
              return Scaffold(
                appBar: AppBar(
                    actions: const [
                      ThemeToggle()
                    ],
                    title: TitleButton(
                        onTap: () => GoRouter.of(context).go('/'),
                        title: settings.title)),
                body: child,
                bottomNavigationBar: AppScreens.bottomNavigationBar(context),
              );
            },
            navigatorKey: shellNavkey,
            parentNavigatorKey: rootNavkey,
            routes: [
              HomeScreen.route(parentNavigatorKey: shellNavkey),
            ]),
      ],
    );
  }

  FutureOr<String?> _guard(BuildContext context, GoRouterState state) {
    String signInRoute = '/';
    final bool signedIn = false;
    final bool signingIn = state.matchedLocation == signInRoute;

    // Go to /signin if the user is not signed in
    if (!signedIn && !signingIn) {
      return signInRoute;
    }
    // Go to / if the user is signed in and tries to go to /auth.
    else if (signedIn && signingIn) {
      return '/';
    }

    // no redirect
    return null;
  }
}
