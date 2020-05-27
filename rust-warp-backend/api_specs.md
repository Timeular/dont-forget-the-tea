Auth: Basic-Auth

`GET /list`
returns all shopping lists ordered by `created_date` desc

`GET /list/id`
returns one shopping list
fields: `name, date_created, items: [item]`

`POST /list`
creates a shopping list
fields: `name, optional date`

`POST /list/id/copy`
creates a new list, based on an old list
fields: `onlyUnchecked boolean (either copies all items, or only unchecked items)`

`DELETE /list/id`
deletes a shopping list

`POST /list/id/item`
adds an item
fields: `name, optional quantity`

`DELETE /list/item/id`
deleted an item

`PUT /list/item/id/check`
checks an item

`PUT /list/item/id/uncheck`
unchecks an item
