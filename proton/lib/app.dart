import 'dart:async';

import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';

import 'proton.dart';

class Proton extends StatefulWidget {
  final AppSettings settings;

  const Proton({Key? key, required this.settings}) : super(key: key);

  @override
  State<Proton> createState() => _ProtonState();
}

class _ProtonState extends State<Proton> {
  @override
  Widget build(BuildContext context) {
    return MaterialApp.router(
      title: widget.settings.title,
      theme: widget.settings.theme,
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
    rootNavkey ??= GlobalKey<NavigatorState>();
    shellNavkey ??= GlobalKey<NavigatorState>();
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
