#!/usr/bin/env bash
# SPDX-License-Identifier: CC0-1.0
# SPDX-FileCopyrightText: NONE

# Install git hooks for development

ln -sfrv scripts/hooks/pre-commit.sh .git/hooks/pre-commit
