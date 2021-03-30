CREATE TABLE task (
    task_id VARCHAR(30) NOT NULL,
    circuit VARCHAR(30) NOT NULL,
    witness TEXT NOT NULL,
    proof TEXT,
    -- status TEXT NOT NULL,
    prover_id VARCHAR(30),
    created_time TIMESTAMP(0) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_time TIMESTAMP(0) NOT NULL ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (task_id)
);

-- CREATE INDEX task_idx_taskid ON task (task_id);
-- CREATE INDEX task_idx_status ON task (status);
-- CREATE UNIQUE INDEX task_prover_constraint ON task (task_id, prover_id);

CREATE TYPE task_status AS ENUM('not_assigned', 'assigned', 'proved');

-- Set task as not_assigned by default
ALTER TABLE task ADD status order_status NOT NULL DEFAULT 'not_assigned';
