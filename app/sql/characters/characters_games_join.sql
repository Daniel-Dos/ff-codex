select c.id AS character_id, c.name AS character_name, g.title AS title, g.release_year AS release_year
from characters c
         INNER JOIN games g on c.game_id = g.id
where g.id = $1