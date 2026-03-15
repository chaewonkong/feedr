# feedr - Project Context

## Overview
Rust로 만드는 RSS/Atom feed reader. Rust 학습이 주 목적이므로 코드 자동 완성보다 단계별 가이드 방식으로 진행.

프로젝트 기능 및 기술 스택은 [README.md](./README.md) 참조.

## Owner
- Go 백엔드 엔지니어 (5년차), Rust 입문자
- Go 대응 개념으로 설명하면 이해가 빠름
- 간결한 설명 + 코드 위주, 과도한 주석 불필요

## Key Decisions
- PostgreSQL over SQLite: k3s 클러스터에 배포 예정, 별도 DB 운영 가능
- sqlx over diesel/sea-orm: raw SQL 스타일, Go database/sql과 유사

## Deployment Target
- 개인 k3s 클러스터 (미니PC 2대, ArgoCD)

## Learning Roadmap
1. 프로젝트 초기화 ✅
2. RSS 파싱 (reqwest + feed-rs) — ownership, Result
3. PostgreSQL 저장 (sqlx) — async, trait
4. 주기적 폴링 (tokio spawn + interval) — async runtime
5. REST API (axum) — extractor, type-based routing
6. Frontend — simple SPA
