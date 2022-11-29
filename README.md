# Requirements Engineering Process (2022-11-28 Pizza Order)
* Rob: Bufala (10.50)
* Hauke: Bufala (10.50) 
* Max: Don Ciro (12.50)
* Felix: Bufala (10.50)

Requirements engineering done. Tasted great.

# Requirements for the muccc food ordering system
## Goals
* Everyone can open a new order
* Orders are public and should be accessible without the need for authentication via a unique URL
* Multiple simultaneous orders
* Multiple simultaneous clients (e.g. a website, matrix bot, etc.)
* have a great name
* countdown timer for the order

## Non Goals
* Authenticated access with users/passwords
* Payment processing

# Decisions

* Backend Interface: GraphQL, REST, gRPC, or other RPC mechanism?
* Database: SQLite or other?
    * => Repository pattern
    * RocksDB afterwards?
* Frontend: Rust! But what framework? Yew?

# Models
## Group Order
### Attributes
* slug: string
* menuUrl: string
* open: boolean
* items: array[OrderEntry]
* (availableViaClients: array[string] # e.g. ["web", "matrix/#rust:darkfasel.net"])
* (date: datetime)
* (pickupTime: datetime)

## Order Entry
### Attributes
* slug: string
* buyer: string
* food: string
* quantity: number (default: 1, min:1) => Do we even want this? We could also have each entry be a single item
* (price: number)
* (paid: boolean)
* (tip: number)

# GRPC Protocol
## Management API
* RegisterClient(clientName: string) => boolean # returns true if the client was registered; false if it already existed
* ClientHeartbeat(clientName: string) => boolean # Returns true if the client is registered and the heartbeat was successful; false otherwise

## Order Service
* NewOrder() => slug
* ToggleOpen(slug)
* GetOrder(slug) => Order

## Single Item Service (for each order)
* AddItem(slug, buyer, food, quantity, price) => slug
* RemoveItem(slug)
* (TogglePaid(slug))

# Debug Commands
* GetOrders() => array[Order]

# Future Ideas
* Easy Ordering View (Summary with only relevant options for phoning the restaurant and checkmarks?)
* Model restaurant menus for frequently used restaurants (with prices, etc.)
