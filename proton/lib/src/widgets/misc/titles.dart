import 'package:flutter/material.dart';

class TitleButton extends StatelessWidget {
  const TitleButton({Key? key, this.onTap, this.title = ''})
      : super(key: key);

  final VoidCallback? onTap;
  final String title;

  @override
  Widget build(BuildContext context) {
    return Tooltip(
        message: title,
        child: InkWell(
          onTap: onTap,
          child: Text(
            title,
          ),
        ));
  }
}
