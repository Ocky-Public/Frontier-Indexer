# Frontier-Indexer
Custom Sui indexer for the EVE Frontier world contracts

## Steps to add migration:

1. Set the PSQL_URL:  
```PSQL_URL=postgres://username@localhost:5432/sui_indexer```

2. Test to ensure connection works:  
```psql $PSQL_URL -c "SELECT 'Connected';"```  
    Expected output:  
    ```
    ?column?
    -----------
    Connected
    (1 row)
    ```

3. Setup connection for cli:  
```diesel setup --database-url $PSQL_URL```

4. Generate your migration:  
```diesel migration generate [name_here]```

5. Run migration::  
```diesel migration run --database-url $PSQL_URL```

6. If `schema.rs` does not update on its own then run:  
```diesel print-schema --database-url $PSQL_URL > src/schema.rs```