ifneq (,$(wildcard .env))
include .env
export
endif

ADMIN_PERSONAL_PERFORMANCE_CONNECTION_STRING=mongodb://${MONGO_INITDB_ROOT_USERNAME}:${MONGO_INITDB_ROOT_PASSWORD}@127.0.0.1:27017/${MONGO_PERSONAL_PERFORMANCE_DB}?authSource=${MONGO_INITDB_DATABASE}
USER_PERSONAL_PERFORMANCE_CONNECTION_STRING=mongodb://${MONGO_PERSONAL_PERFORMANCE_USER}:${MONGO_PERSONAL_PERFORMANCE_PASSWORD}@127.0.0.1:27017/${MONGO_PERSONAL_PERFORMANCE_DB}?authSource=${MONGO_PERSONAL_PERFORMANCE_DB}

DOCKER_EXECUTE_STRING=docker exec -it personal-performance-db /bin/bash -c


db-create-user:
	${DOCKER_EXECUTE_STRING} "mongosh ${ADMIN_PERSONAL_PERFORMANCE_CONNECTION_STRING} --eval 'db.createUser({user: \"${MONGO_PERSONAL_PERFORMANCE_USER}\", pwd: \"${MONGO_PERSONAL_PERFORMANCE_PASSWORD}\", roles: [{role: \"readWrite\", db: \"${MONGO_PERSONAL_PERFORMANCE_DB}\"}]})'" 

db-connect-admin:
	${DOCKER_EXECUTE_STRING} "mongosh ${ADMIN_PERSONAL_PERFORMANCE_CONNECTION_STRING}"

db-connect-user:
	${DOCKER_EXECUTE_STRING} "mongosh ${USER_PERSONAL_PERFORMANCE_CONNECTION_STRING}"

unit-tests:
	cargo llvm-cov --workspace --ignore-filename-regex="test_utils|main" --all-features -- --test-threads=1 --nocapture

fmt-check:
	cargo fmt --all --check

clippy-check:
	cargo clippy -- --D warnings

watch:
	cargo watch -q -w src -x run
