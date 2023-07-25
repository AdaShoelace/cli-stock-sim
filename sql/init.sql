-- Table: public.stocks

-- DROP TABLE IF EXISTS public.stocks;

CREATE TABLE IF NOT EXISTS public.stocks
(
    id integer NOT NULL DEFAULT nextval('stocks_id_seq'::regclass),
    name character varying(128) COLLATE pg_catalog."default" NOT NULL,
    industry_id bigint NOT NULL,
    created_on timestamp without time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_on timestamp without time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT stocks_pkey PRIMARY KEY (id),
    CONSTRAINT stocks_name_key UNIQUE (name)
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.stocks
    OWNER to "stock-dev-user";

-- FUNCTION: public.update_updated_on_column()

-- DROP FUNCTION IF EXISTS public.update_updated_on_column();

CREATE OR REPLACE FUNCTION public.update_updated_on_column()
    RETURNS trigger
    LANGUAGE 'plpgsql'
    COST 100
    VOLATILE NOT LEAKPROOF
AS $BODY$
BEGIN
   NEW.updated_on = now(); 
   RETURN NEW;
END;
$BODY$;

ALTER FUNCTION public.update_updated_on_column()
    OWNER TO "stock-dev-user";
-- Trigger: update_updated_on

-- DROP TRIGGER IF EXISTS update_updated_on ON public.stocks;

CREATE OR REPLACE TRIGGER update_updated_on_stocks
    BEFORE UPDATE 
    ON public.stocks
    FOR EACH ROW
    EXECUTE FUNCTION public.update_updated_on_column();

-------------------------------------------------------

-- Table: public.users

-- DROP TABLE IF EXISTS public.users;

CREATE TABLE IF NOT EXISTS public.users
(
    id bigint NOT NULL DEFAULT nextval('users_id_seq'::regclass),
    name character varying(128) COLLATE pg_catalog."default" NOT NULL,
    display_name character varying(128) COLLATE pg_catalog."default" NOT NULL,
    created_on timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    updated_on timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT users_pkey PRIMARY KEY (id),
    CONSTRAINT users_display_name_key UNIQUE (display_name)
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.users
    OWNER to "stock-dev-user";

-- Trigger: update_updated_on_users

-- DROP TRIGGER IF EXISTS update_updated_on_users ON public.users;

CREATE OR REPLACE TRIGGER update_updated_on_users
    BEFORE UPDATE 
    ON public.users
    FOR EACH ROW
    EXECUTE FUNCTION public.update_updated_on_column();

----------------------------------------------------------

-- Table: public.industries

-- DROP TABLE IF EXISTS public.industries;

CREATE TABLE IF NOT EXISTS public.industries
(
    id integer NOT NULL DEFAULT nextval('industries_id_seq'::regclass),
    name character varying(128) COLLATE pg_catalog."default" NOT NULL,
    created_on timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    updated_on timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT industries_pkey PRIMARY KEY (id)
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.industries
    OWNER to "stock-dev-user";

-- Trigger: update_updated_on_industries

-- DROP TRIGGER IF EXISTS update_updated_on_industries ON public.industries;

CREATE OR REPLACE TRIGGER update_updated_on_industries
    BEFORE UPDATE 
    ON public.industries
    FOR EACH ROW
    EXECUTE FUNCTION public.update_updated_on_column();

------------------------------------------------------------------

-- View: public.stock_view

-- DROP VIEW public.stock_view;

CREATE OR REPLACE VIEW public.stock_view
 AS
 SELECT stocks.name,
    stocks.industry_id,
    industries.name AS industry_name
   FROM stocks
     LEFT JOIN industries ON stocks.industry_id = industries.id::bigint;

ALTER TABLE public.stock_view
    OWNER TO "stock-dev-user";

