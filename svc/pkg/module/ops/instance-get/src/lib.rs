use proto::backend::{self, pkg::*};
use rivet_operation::prelude::*;

#[derive(sqlx::FromRow)]
struct Instance {
	instance_id: Uuid,
	version_id: Uuid,
	create_ts: i64,
	destroy_ts: Option<i64>,

	driver_dummy: bool,

	driver_fly: bool,
	driver_fly_app_id: Option<String>,
}

#[operation(name = "module-instance-get")]
pub async fn handle(
	ctx: OperationContext<module::instance_get::Request>,
) -> GlobalResult<module::instance_get::Response> {
	let instance_ids = ctx
		.instance_ids
		.iter()
		.map(common::Uuid::as_uuid)
		.collect::<Vec<_>>();

	let instances = sql_fetch_all!(
		[ctx, Instance]
		"
		SELECT
			i.instance_id,
			i.version_id,
			i.create_ts,
			i.destroy_ts,
			idd.instance_id IS NOT NULL AS driver_dummy,
			idv.instance_id IS NOT NULL AS driver_fly,
			idv.fly_app_id AS driver_fly_app_id
		FROM db_module.instances AS i
		LEFT JOIN db_module.instances_driver_dummy AS idd ON idd.instance_id = i.instance_id
		LEFT JOIN db_module.instances_driver_fly AS idv ON idv.instance_id = i.instance_id
		WHERE i.instance_id = ANY($1)
		",
		&instance_ids,
	)
	.await?;

	Ok(module::instance_get::Response {
		instances: instances
			.into_iter()
			.map(|instance| {
				let driver = if instance.driver_dummy {
					backend::module::instance::Driver::Dummy(backend::module::instance::Dummy {})
				} else if instance.driver_fly {
					backend::module::instance::Driver::Fly(backend::module::instance::Fly {
						fly_app_id: instance.driver_fly_app_id,
					})
				} else {
					bail!("instance has no driver")
				};

				GlobalResult::Ok(backend::module::Instance {
					instance_id: Some(instance.instance_id.into()),
					module_version_id: Some(instance.version_id.into()),
					create_ts: instance.create_ts,
					destroy_ts: instance.destroy_ts,
					driver: Some(driver),
				})
			})
			.collect::<GlobalResult<Vec<_>>>()?,
	})
}
