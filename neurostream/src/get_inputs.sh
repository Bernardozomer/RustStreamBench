#!/bin/bash

mkdir inputs
curl https://archive.ics.uci.edu/ml/machine-learning-databases/image/segmentation.data --output inputs/segmentation.data
curl https://archive.ics.uci.edu/ml/machine-learning-databases/image/segmentation.names --output inputs/segmentation.names
curl https://archive.ics.uci.edu/ml/machine-learning-databases/image/segmentation.test --output inputs/segmentation.test

