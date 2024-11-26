ENTITIES_SRC_DIR=napoli-server-persistent-entities/src
rm -rf $ENTITIES_SRC_DIR
sea-orm-cli generate entity -o $ENTITIES_SRC_DIR -u "sqlite://napoli.sqlite" --lib
