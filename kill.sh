#!/usr/bin/env bash
ps uax|grep debug/y|grep -v grep|awk '{print $2}'|xargs kill -9
