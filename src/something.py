def get_consumption_for_meter_point_in_period(
    meter_point_id: int,
    start_at: datetime.datetime,
    end_at: datetime.datetime,
    create_estimates: bool,
) -> service.MeterPointConsumption:
    """
    Retrieves the consumption for a given meter point over a specified period.
    Args:
        meter_point_id (int): The ID of the meter point.
        start_at (datetime.datetime): The start date and time of the period.
        end_at (datetime.datetime): The end date and time of the period.
        create_estimates (bool): Whether to create estimates if readings are not available on the given dates.
    Returns:
        MeterPointConsumption
    Notes:
        Currently, create_estimates is not implemented and will only return consumption based on if the exact
        reading dates exist.
    """
    meter_finder = adapters.MeterFinder()
    reading_finder = adapters.ReadingFinder()
    estimation_service = service.EstimationService(
        meter_finder=meter_finder,
        reading_finder=reading_finder,
        consumption_repository=consumption_reading.DjangoConsumptionReadingRepository(),
    )

    return estimation_service.calculate_consumption(
        meter_point_id=meter_point_id,
        start_at=start_at,
        end_at=end_at,
        create_estimates=create_estimates,
    )


class GetConsmptionForMeterPointInPeriod:
    def __init__(
        self,
        meter_finder: MeterFinderProtocol,
        reading_finder: ReadingFinderProtocol,
        consumption_repository: ConsumptionReadingRepository,
    ):
        self._meter_finder = meter_finder
        self._reading_finder = reading_finder
        self._consumption_repository = consumption_repository

    def __call__(
        self,
        meter_point_id: int,
        start_at: datetime.datetime,
        end_at: datetime.datetime,
        create_estimates: bool,
    ):
        estimation_service = EstimationService(
            meter_finder=self._meter_finder,
            reading_finder=self._reading_finder,
            consumption_repository=self._consumption_repository,
        )

        return estimation_service.calculate_consumption(
            meter_point_id=meter_point_id,
            start_at=start_at,
            end_at=end_at,
            create_estimates=create_estimates,
        )

get_consumption_for_meter_point_in_period = GetConsmptionForMeterPointInPeriod(
    meter_finder: MeterFinder(),
    reading_finder: ReadingFinder(),
    consumption_repository: ConsumptionReadingRepository(),
)
