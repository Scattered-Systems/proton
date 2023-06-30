import 'package:flutter/material.dart';

/// [Loader] is a base class used to
abstract class Loader<T extends Object?> {
  final Widget Function(BuildContext, T) builder;
  final T? initialData;
  final Future<T>? future;
  final bool sliver;
  final Stream<T>? stream;

  const Loader(
      {required this.builder,
      this.future,
      this.initialData,
      this.sliver = false,
      this.stream})
      : assert(future != null || stream != null);

  Widget loader(BuildContext context, AsyncSnapshot<T> snapshot);

  FutureBuilder<T> futureBuilder(Future<T> future);

  StreamBuilder<T> streamBuilder(Stream<T> stream);
}

mixin LoaderMixin<T extends Object?> on Loader<T> {
  @override
  FutureBuilder<T> futureBuilder(Future<T> future) {
    return FutureBuilder<T>(
      future: future,
      initialData: initialData,
      builder: loader,
    );
  }

  @override
  StreamBuilder<T> streamBuilder(Stream<T> stream) {
    return StreamBuilder<T>(
      initialData: initialData,
      stream: stream,
      builder: loader,
    );
  }
}

/// [AsyncBuilder] is a unifed widget that can be used to build itself based on
/// the latest snapshot of interaction with a [Future] or [Stream].
/// Additionally, the [AsyncBuilder] can be used within a [CustomScrollView]
/// using the [sliver] property to toggle between a [SliverToBoxAdapter] and
/// a [Center] widget.
class AsyncBuilder<T extends Object?> extends StatelessWidget {
  final Widget Function(BuildContext, T) builder;
  final T? initialData;
  final Future<T>? future;
  final bool sliver;
  final Stream<T>? stream;

  const AsyncBuilder({
    Key? key,
    required this.builder,
    this.future,
    this.initialData,
    this.sliver = false,
    this.stream,
  })  : assert(future != null || stream != null,
            'Missing an asynchrous operation...'),
        super(key: key);

  @override
  Widget build(BuildContext context) {
    if (future != null) {
      return FutureBuilder<T>(
        future: future,
        initialData: initialData,
        builder: loader,
      );
    } else {
      return StreamBuilder<T>(
        initialData: initialData,
        stream: stream,
        builder: loader,
      );
    }
  }

  Widget loader(BuildContext context, AsyncSnapshot<T> snapshot) {
    if (snapshot.hasError) {
      return sliverAdapter(child: Text(snapshot.error!.toString()));
    }
    if (snapshot.connectionState == ConnectionState.waiting) {
      return sliverAdapter(child: const CircularProgressIndicator());
    }
    if (snapshot.hasData) {
      return builder(context, snapshot.requireData);
    }
    return sliverAdapter(child: const Text('Nothing found...'));
  }

  Widget sliverAdapter({required Widget child}) {
    final Widget inner = Center(child: child);
    return sliver ? SliverToBoxAdapter(child: inner) : Center(child: inner);
  }
}
