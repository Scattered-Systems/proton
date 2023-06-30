import 'package:flutter/material.dart';




mixin ItemMixin<T> {
  Widget get item;

  static ListView listView<T extends ItemMixin>(List<T> items) {
    return ListView.builder(
        itemCount: items.length,
        itemBuilder: (context, index) {
          return items[index].item;
        });
  }
}
