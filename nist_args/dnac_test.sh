#! /usr/bin/env bash

for input in $(ls dnac/*_1.txt); do
        ./assess 1048576 < $input
        mv experiments/AlgorithmTesting/finalAnalysisReport.txt dnac/finalAnalysisReport$input
done

for input in $(ls dnac/*_2.txt); do
        ./assess 1056896 < $input
        mv experiments/AlgorithmTesting/finalAnalysisReport.txt dnac/finalAnalysisReport$input
done
