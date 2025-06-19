import 'package:env_comparison/utils/colors.dart';
import 'package:fl_chart/fl_chart.dart';
import 'package:flex_color_scheme/flex_color_scheme.dart';
import 'package:flutter/material.dart';

class ResultChart extends StatefulWidget {
  ResultChart({super.key});

  final Color dark = Colors.cyan.darken(60);
  final Color normal = Colors.cyan.darken(30);
  final Color light = Colors.cyan;

  @override
  State<StatefulWidget> createState() => ResultChartState();
}

class ResultChartState extends State<ResultChart> {
  Widget bottomTitles(double value, TitleMeta meta) {
    const style = TextStyle(fontSize: 10);
    String text;
    switch (value.toInt()) {
      case 0:
        text = 'Topic';
      case 1:
        text = 'Share Class';
      case 2:
        text = 'Media Outlet Map';
      default:
        text = 'Default';
    }
    return SideTitleWidget(
      meta: meta,
      child: Text(text, style: style),
    );
  }

  Widget leftTitles(double value, TitleMeta meta) {
    if (value == meta.max) {
      return Container();
    }
    const style = TextStyle(fontSize: 10);
    return SideTitleWidget(
      meta: meta,
      child: Text(meta.formattedValue, style: style),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Stack(
      children: [
        Positioned(
          top: 0,
          right: 8,
          child: Column(
            children: [
              Row(
                children: [
                  Container(width: 10, height: 10, color: AppColors.add),
                  SizedBox(width: 5),
                  Text('Adds', style: TextStyle(fontSize: 12)),
                ],
              ),
              SizedBox(height: 8),
              Row(
                children: [
                  Container(width: 10, height: 10, color: AppColors.mod),
                  SizedBox(width: 5),
                  Text('Mods', style: TextStyle(fontSize: 12)),
                ],
              ),
              SizedBox(height: 8),
              Row(
                children: [
                  Container(width: 10, height: 10, color: AppColors.del),
                  SizedBox(width: 5),
                  Text('Dels', style: TextStyle(fontSize: 12)),
                ],
              ),
            ],
          ),
        ),
        AspectRatio(
          aspectRatio: 3,
          child: Padding(
            padding: const EdgeInsets.only(top: 16, left: 50),
            child: LayoutBuilder(
              builder: (context, constraints) {
                final barsSpace = 4.0 * constraints.maxWidth / 100;
                final barsWidth = 8.0 * constraints.maxWidth / 400;
                return BarChart(
                  BarChartData(
                    alignment: BarChartAlignment.start,
                    barTouchData: const BarTouchData(enabled: false),
                    titlesData: FlTitlesData(
                      show: true,
                      bottomTitles: AxisTitles(
                        sideTitles: SideTitles(
                          showTitles: true,
                          reservedSize: 28,
                          getTitlesWidget: bottomTitles,
                        ),
                      ),
                      leftTitles: AxisTitles(
                        sideTitles: SideTitles(
                          showTitles: false,
                          reservedSize: 40,
                          getTitlesWidget: leftTitles,
                        ),
                      ),
                      topTitles: const AxisTitles(
                        sideTitles: SideTitles(showTitles: false),
                      ),
                      rightTitles: const AxisTitles(
                        sideTitles: SideTitles(showTitles: false),
                      ),
                    ),
                    gridData: FlGridData(
                      show: true,
                      checkToShowHorizontalLine: (value) => value % 10 == 0,
                      getDrawingHorizontalLine: (value) => FlLine(
                        color: Colors.red.withValues(alpha: 0.1),
                        strokeWidth: 1,
                      ),
                      drawVerticalLine: false,
                    ),
                    borderData: FlBorderData(show: false),
                    // groupsSpace: barsSpace,
                    groupsSpace: 50,
                    barGroups: getData(barsWidth, barsSpace),
                  ),
                );
              },
            ),
          ),
        ),
      ],
    );
  }

  List<BarChartGroupData> getData(double barsWidth, double barsSpace) {
    return [
      BarChartGroupData(
        x: 0,
        barsSpace: 5,
        showingTooltipIndicators: [0, 1, 2],
        barRods: [
          BarChartRodData(
            toY: 1,
            rodStackItems: [BarChartRodStackItem(0, 1, AppColors.add)],
            borderRadius: BorderRadius.zero,
            width: barsWidth,
          ),
          BarChartRodData(
            toY: 3,
            rodStackItems: [BarChartRodStackItem(0, 3, AppColors.mod)],
            borderRadius: BorderRadius.zero,
            width: barsWidth,
          ),
          BarChartRodData(
            toY: 2,
            rodStackItems: [BarChartRodStackItem(0, 2, AppColors.del)],
            borderRadius: BorderRadius.zero,
            width: barsWidth,
          ),
        ],
      ),
      BarChartGroupData(
        x: 1,
        showingTooltipIndicators: [0],
        barsSpace: barsSpace,
        barRods: [],
      ),
      BarChartGroupData(
        x: 2,
        barsSpace: 5,
        showingTooltipIndicators: [2],
        barRods: [
          BarChartRodData(
            toY: 0,
            rodStackItems: [BarChartRodStackItem(0, 0, AppColors.add)],
            borderRadius: BorderRadius.zero,
            width: barsWidth,
          ),
          BarChartRodData(
            toY: 0,
            rodStackItems: [BarChartRodStackItem(0, 0, AppColors.mod)],
            borderRadius: BorderRadius.zero,
            width: barsWidth,
          ),
          BarChartRodData(
            toY: 6,
            rodStackItems: [BarChartRodStackItem(0, 6, AppColors.del)],
            borderRadius: BorderRadius.zero,
            width: barsWidth,
          ),
        ],
      ),
    ];
  }
}
